import { useEffect } from "react";
import { useSubscriptionStore } from "@/store/subscriptionStore";
import { useAuth } from "./useAuth";

export function useSubscription() {
  const { isAuthenticated } = useAuth();
  const { subscription, setSubscription, isSubscriber } = useSubscriptionStore();

  useEffect(() => {
    if (!isAuthenticated) return;

    fetch("/api/subscription")
      .then((r) => r.json())
      .then((data) => {
        if (data.subscription) setSubscription(data.subscription);
      })
      .catch(() => {});
  }, [isAuthenticated, setSubscription]);

  return { subscription, isSubscriber: isSubscriber() };
}
