import Head from "next/head";
import { useRouter } from "next/router";
import { AuthGuard } from "@/components/auth/AuthGuard";
import { Button } from "@/components/ui/Button";
import { Badge } from "@/components/ui/Badge";
import { useAuth } from "@/hooks/useAuth";
import { useSubscription } from "@/hooks/useSubscription";

function DashboardContent() {
  const router = useRouter();
  const { user, signOut } = useAuth();
  const { subscription } = useSubscription();

  return (
    <>
      <Head>
        <title>Dashboard — ontext</title>
      </Head>

      <main className="min-h-screen bg-gray-50 px-4 py-12">
        <div className="mx-auto max-w-2xl">
          <div className="mb-8 flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
              <p className="mt-1 text-sm text-gray-500">{user?.email}</p>
            </div>
            <Button variant="outline" onClick={() => signOut()}>
              Sign out
            </Button>
          </div>

          {router.query.checkout === "success" && (
            <div className="mb-6 rounded-xl bg-green-50 p-4 text-sm text-green-700">
              Subscription activated — welcome to ontext Pro!
            </div>
          )}

          <div className="rounded-2xl bg-white p-6 ring-1 ring-gray-200">
            <h2 className="mb-4 text-base font-semibold text-gray-900">
              Your plan
            </h2>
            <div className="flex items-center gap-3">
              <Badge
                variant={
                  subscription.planId === "pro"
                    ? "pro"
                    : subscription.planId === "team"
                    ? "team"
                    : "default"
                }
              >
                {subscription.planId.charAt(0).toUpperCase() +
                  subscription.planId.slice(1)}
              </Badge>
              <Badge
                variant={
                  subscription.status === "active" ||
                  subscription.status === "trialing"
                    ? "success"
                    : subscription.status === "past_due"
                    ? "warning"
                    : "default"
                }
              >
                {subscription.status === "none" ? "inactive" : subscription.status}
              </Badge>
            </div>

            {subscription.currentPeriodEnd && (
              <p className="mt-3 text-sm text-gray-500">
                Renews{" "}
                {new Date(subscription.currentPeriodEnd).toLocaleDateString()}
              </p>
            )}

            {subscription.planId === "free" && (
              <div className="mt-4">
                <Button onClick={() => router.push("/pricing")}>
                  Upgrade to Pro
                </Button>
              </div>
            )}
          </div>
        </div>
      </main>
    </>
  );
}

export default function DashboardPage() {
  return (
    <AuthGuard requireSubscriber={false}>
      <DashboardContent />
    </AuthGuard>
  );
}
