import { withAuth } from "next-auth/middleware";

export default withAuth({
  pages: {
    signIn: "/login",
  },
});

// Protect only these routes at the edge — client-side AuthGuard handles
// subscriber checks, which require a DB call (not suitable for edge).
export const config = {
  matcher: ["/dashboard/:path*"],
};
