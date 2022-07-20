import { useSession, signIn, signOut } from "next-auth/react"


export default function LoginButton() {
    const { data: session } = useSession()
    if(session) {
        return <div>
            Signed in as {session?.user?.email} <br/>
            <button onClick={() => signOut()}>Sign out</button>
        </div>
    }
    return <>
        <div className="bg-blue-600 hover:bg-blue-500 border-gray-50 items-center py-1 px-3 border-b-2 border-l-2 hover:text-slate-400 hover:font-semibold">
            <button onClick={() => signIn()}>
                Log In
            </button>
        </div>
    </>
}