import Head from "next/head";
import { useRouter } from "next/router";
import { PricingTable } from "@/components/pricing/PricingTable";
import { Button } from "@/components/ui/Button";
import { useSubscription } from "@/hooks/useSubscription";
import { useCheckout } from "@/hooks/useCheckout";
import { useAuth } from "@/hooks/useAuth";
import type { PlanId } from "@/store/subscriptionStore";

export default function PricingPage() {
  const router = useRouter();
  const { isAuthenticated, signIn } = useAuth();
  const { subscription } = useSubscription();
  const { startCheckout, loading, error } = useCheckout();

  async function handleSelect(planId: PlanId) {
    if (planId === "free") return;
    if (!isAuthenticated) {
      await signIn();
      return;
    }
    await startCheckout(planId);
  }

  return (
    <>
      <Head>
        <title>Pricing — ontext</title>
      </Head>

      <main className="min-h-screen bg-white px-4 py-24">
        <div className="mx-auto max-w-5xl">
          <div className="mb-16 text-center">
            <h1 className="text-4xl font-extrabold tracking-tight text-gray-900 sm:text-5xl">
              Simple pricing
            </h1>
            <p className="mt-4 text-lg text-gray-500">
              Transcribe hands-free on every platform. Upgrade anytime.
            </p>
            {router.query.reason === "subscription-required" && (
              <p className="mt-4 text-sm font-medium text-amber-600">
                A subscription is required to access that page.
              </p>
            )}
            {router.query.checkout === "canceled" && (
              <p className="mt-4 text-sm font-medium text-gray-500">
                Checkout was canceled. Your plan was not changed.
              </p>
            )}
          </div>

          {error && (
            <p className="mb-8 text-center text-sm text-red-600">{error}</p>
          )}

          <PricingTable
            currentPlanId={subscription.planId}
            onSelect={handleSelect}
            checkoutLoading={loading}
          />

          {!isAuthenticated && (
            <p className="mt-12 text-center text-sm text-gray-500">
              Already have an account?{" "}
              <Button variant="ghost" onClick={() => signIn()} className="p-0 text-sm">
                Sign in
              </Button>
            </p>
          )}
        </div>
      </main>
    </>
  );
}
