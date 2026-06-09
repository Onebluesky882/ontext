import { useRouter } from "next/router";
import { useEffect } from "react";
import { useAuth } from "@/hooks/useAuth";
import { useSubscription } from "@/hooks/useSubscription";

type Props = {
  requireSubscriber?: boolean;
  children: React.ReactNode;
};

export function AuthGuard({ requireSubscriber = false, children }: Props) {
  const router = useRouter();
  const { isAuthenticated, isLoading } = useAuth();
  const { isSubscriber } = useSubscription();

  useEffect(() => {
    if (isLoading) return;
    if (!isAuthenticated) {
      router.replace(`/login?callbackUrl=${encodeURIComponent(router.asPath)}`);
      return;
    }
    if (requireSubscriber && !isSubscriber) {
      router.replace("/pricing?reason=subscription-required");
    }
  }, [isLoading, isAuthenticated, requireSubscriber, isSubscriber, router]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="h-8 w-8 animate-spin rounded-full border-4 border-brand-500 border-t-transparent" />
      </div>
    );
  }

  if (!isAuthenticated) return null;
  if (requireSubscriber && !isSubscriber) return null;

  return <>{children}</>;
}
