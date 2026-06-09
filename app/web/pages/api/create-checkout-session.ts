import type { NextApiRequest, NextApiResponse } from "next";
import Stripe from "stripe";
import { getServerSession } from "next-auth";
import { authOptions } from "./auth/[...nextauth]";
import type { PlanId } from "@/store/subscriptionStore";

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY!, {
  apiVersion: "2024-04-10",
});

const PRICE_IDS: Record<Exclude<PlanId, "free">, string> = {
  pro: process.env.STRIPE_PRO_PRICE_ID!,
  team: process.env.STRIPE_TEAM_PRICE_ID!,
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  const session = await getServerSession(req, res, authOptions);
  if (!session?.user?.email) {
    return res.status(401).json({ error: "Not authenticated" });
  }

  const { planId } = req.body as { planId: PlanId };
  if (planId === "free" || !PRICE_IDS[planId as Exclude<PlanId, "free">]) {
    return res.status(400).json({ error: "Invalid plan" });
  }

  const checkoutSession = await stripe.checkout.sessions.create({
    mode: "subscription",
    payment_method_types: ["card"],
    customer_email: session.user.email,
    line_items: [
      {
        price: PRICE_IDS[planId as Exclude<PlanId, "free">],
        quantity: 1,
      },
    ],
    success_url: `${process.env.NEXTAUTH_URL}/dashboard?checkout=success`,
    cancel_url: `${process.env.NEXTAUTH_URL}/pricing?checkout=canceled`,
    metadata: { planId, userEmail: session.user.email },
  });

  return res.status(200).json({ sessionId: checkoutSession.id });
}
