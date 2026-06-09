import { PricingCard, type Plan } from "./PricingCard";
import type { PlanId } from "@/store/subscriptionStore";

const PLANS: Plan[] = [
  {
    id: "free",
    name: "Free",
    price: "$0",
    period: "/ month",
    description: "Try ontext at no cost.",
    features: [
      "30 transcriptions / month",
      "Standard accuracy",
      "macOS & Windows",
    ],
  },
  {
    id: "pro",
    name: "Pro",
    price: "$9",
    period: "/ month",
    description: "For individuals who transcribe daily.",
    features: [
      "Unlimited transcriptions",
      "Priority Whisper API",
      "All platforms (iOS & Android)",
      "Custom hotkeys",
      "Email support",
    ],
    highlighted: true,
    badge: "Most popular",
  },
  {
    id: "team",
    name: "Team",
    price: "$29",
    period: "/ month",
    description: "For teams that need shared billing.",
    features: [
      "Everything in Pro",
      "Up to 10 seats",
      "Admin dashboard",
      "Priority support",
      "SSO (coming soon)",
    ],
    badge: "Best value",
  },
];

type Props = {
  currentPlanId: PlanId;
  onSelect: (planId: PlanId) => void;
  checkoutLoading?: boolean;
};

export function PricingTable({ currentPlanId, onSelect, checkoutLoading }: Props) {
  return (
    <div className="grid gap-8 md:grid-cols-3">
      {PLANS.map((plan) => (
        <PricingCard
          key={plan.id}
          plan={plan}
          currentPlanId={currentPlanId}
          onSelect={onSelect}
          loading={checkoutLoading}
        />
      ))}
    </div>
  );
}
