import { Button } from "@/components/ui/Button";
import { Badge } from "@/components/ui/Badge";
import type { PlanId } from "@/store/subscriptionStore";

export type PlanFeature = string;

export type Plan = {
  id: PlanId;
  name: string;
  price: string;
  period: string;
  description: string;
  features: PlanFeature[];
  highlighted?: boolean;
  badge?: string;
};

type Props = {
  plan: Plan;
  currentPlanId: PlanId;
  onSelect: (planId: PlanId) => void;
  loading?: boolean;
};

export function PricingCard({ plan, currentPlanId, onSelect, loading }: Props) {
  const isCurrent = plan.id === currentPlanId;

  return (
    <div
      className={`relative flex flex-col rounded-2xl border p-8 ${
        plan.highlighted
          ? "border-brand-500 shadow-lg shadow-brand-500/10"
          : "border-gray-200"
      }`}
    >
      {plan.badge && (
        <div className="absolute -top-3.5 left-1/2 -translate-x-1/2">
          <Badge variant={plan.id === "pro" ? "pro" : "team"}>
            {plan.badge}
          </Badge>
        </div>
      )}

      <div className="mb-6">
        <h3 className="text-lg font-bold text-gray-900">{plan.name}</h3>
        <p className="mt-1 text-sm text-gray-500">{plan.description}</p>
        <div className="mt-4 flex items-baseline gap-1">
          <span className="text-4xl font-extrabold text-gray-900">
            {plan.price}
          </span>
          <span className="text-sm text-gray-500">{plan.period}</span>
        </div>
      </div>

      <ul className="mb-8 flex-1 space-y-3">
        {plan.features.map((f) => (
          <li key={f} className="flex items-start gap-2 text-sm text-gray-600">
            <svg
              className="mt-0.5 h-4 w-4 shrink-0 text-brand-500"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fillRule="evenodd"
                d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                clipRule="evenodd"
              />
            </svg>
            {f}
          </li>
        ))}
      </ul>

      <Button
        variant={plan.highlighted ? "primary" : "outline"}
        loading={loading && !isCurrent}
        disabled={isCurrent}
        onClick={() => onSelect(plan.id)}
        className="w-full"
      >
        {isCurrent ? "Current plan" : plan.id === "free" ? "Get started" : "Upgrade"}
      </Button>
    </div>
  );
}
