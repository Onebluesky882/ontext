import type { NextApiRequest, NextApiResponse } from "next";
import { getServerSession } from "next-auth";
import { authOptions } from "./auth/[...nextauth]";
import type { Subscription } from "@/store/subscriptionStore";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "GET") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  const session = await getServerSession(req, res, authOptions);
  if (!session?.user?.email) {
    return res.status(401).json({ error: "Not authenticated" });
  }

  // TODO: query real DB for subscription by session.user.email
  const subscription: Subscription = {
    planId: "free",
    status: "none",
    currentPeriodEnd: null,
    stripeCustomerId: null,
  };

  return res.status(200).json({ subscription });
}
