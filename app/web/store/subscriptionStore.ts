import { create } from "zustand";
import { persist } from "zustand/middleware";

export type PlanId = "free" | "pro" | "team";
export type SubscriptionStatus =
  | "active"
  | "trialing"
  | "past_due"
  | "canceled"
  | "none";

export type Subscription = {
  planId: PlanId;
  status: SubscriptionStatus;
  currentPeriodEnd: string | null;
  stripeCustomerId: string | null;
};

type SubscriptionState = {
  subscription: Subscription;
  setSubscription: (sub: Subscription) => void;
  isSubscriber: () => boolean;
};

const DEFAULT: Subscription = {
  planId: "free",
  status: "none",
  currentPeriodEnd: null,
  stripeCustomerId: null,
};

export const useSubscriptionStore = create<SubscriptionState>()(
  persist(
    (set, get) => ({
      subscription: DEFAULT,
      setSubscription: (subscription) => set({ subscription }),
      isSubscriber: () => {
        const { status } = get().subscription;
        return status === "active" || status === "trialing";
      },
    }),
    { name: "ontext-subscription" }
  )
);
