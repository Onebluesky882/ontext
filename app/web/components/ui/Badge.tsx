type Variant = "default" | "pro" | "team" | "success" | "warning";

type Props = {
  variant?: Variant;
  children: React.ReactNode;
};

const variantClass: Record<Variant, string> = {
  default: "bg-gray-100 text-gray-700",
  pro: "bg-brand-50 text-brand-700",
  team: "bg-violet-50 text-violet-700",
  success: "bg-green-50 text-green-700",
  warning: "bg-amber-50 text-amber-700",
};

export function Badge({ variant = "default", children }: Props) {
  return (
    <span
      className={`inline-block rounded-full px-3 py-0.5 text-xs font-semibold ${variantClass[variant]}`}
    >
      {children}
    </span>
  );
}
