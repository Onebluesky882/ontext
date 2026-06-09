import { useState } from "react";
import { loadStripe } from "@stripe/stripe-js";
import type { PlanId } from "@/store/subscriptionStore";

const stripePromise = loadStripe(
  process.env.NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY!
);

export function useCheckout() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function startCheckout(planId: PlanId) {
    if (planId === "free") return;
    setLoading(true);
    setError(null);

    try {
      const res = await fetch("/api/create-checkout-session", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ planId }),
      });

      if (!res.ok) {
        const { error: msg } = await res.json();
        throw new Error(msg ?? "Failed to create checkout session");
      }

      const { sessionId } = await res.json();
      const stripe = await stripePromise;
      if (!stripe) throw new Error("Stripe not loaded");
      await stripe.redirectToCheckout({ sessionId });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }

  return { startCheckout, loading, error };
}
