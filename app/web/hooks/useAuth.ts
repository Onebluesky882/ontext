import { useSession, signIn, signOut } from "next-auth/react";
import { useEffect } from "react";
import { useAuthStore } from "@/store/authStore";

export function useAuth() {
  const { data: session, status } = useSession();
  const { user, setUser } = useAuthStore();

  useEffect(() => {
    if (session?.user) {
      setUser({
        id: (session.user as { id?: string }).id ?? session.user.email ?? "",
        email: session.user.email ?? "",
        name: session.user.name ?? null,
      });
    } else if (status === "unauthenticated") {
      setUser(null);
    }
  }, [session, status, setUser]);

  return {
    user,
    isLoading: status === "loading",
    isAuthenticated: status === "authenticated",
    signIn: (provider?: string) => signIn(provider),
    signOut: () => signOut({ callbackUrl: "/" }),
  };
}
