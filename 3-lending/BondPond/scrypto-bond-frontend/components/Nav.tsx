function Links() {
    return (
        <>
            <a href="/about" className="px-3 hover:text-slate-400 hover:font-semibold">About</a>
        </>
    )
}

export default function Nav() {
    return (
        <nav className="pr-4">
            <Links />
        </nav> 
    )
}