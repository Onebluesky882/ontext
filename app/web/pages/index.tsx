import Head from "next/head";
import { useRouter } from "next/router";
import { Button } from "@/components/ui/Button";
import { useAuth } from "@/hooks/useAuth";

export default function HomePage() {
  const router = useRouter();
  const { isAuthenticated } = useAuth();

  return (
    <>
      <Head>
        <title>ontext — Speech to text, instantly</title>
      </Head>

      <main className="flex min-h-screen flex-col items-center justify-center bg-white px-4 text-center">
        <h1 className="text-5xl font-extrabold tracking-tight text-gray-900 sm:text-6xl">
          ontext
        </h1>
        <p className="mt-4 max-w-xl text-lg text-gray-500">
          Press a hotkey, speak, release — your words appear wherever you are
          typing. No switching apps, no clicking.
        </p>

        <div className="mt-10 flex gap-4">
          <Button onClick={() => router.push("/pricing")}>See pricing</Button>
          {isAuthenticated ? (
            <Button variant="outline" onClick={() => router.push("/dashboard")}>
              Dashboard
            </Button>
          ) : (
            <Button variant="outline" onClick={() => router.push("/login")}>
              Sign in
            </Button>
          )}
        </div>
      </main>
    </>
  );
}
