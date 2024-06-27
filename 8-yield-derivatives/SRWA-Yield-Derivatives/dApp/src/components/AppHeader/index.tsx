function AppHeader() {
  return (
    <header className="bg-white border-b border-slate-300">
      <nav className="mx-auto flex max-w-7xl items-center justify-between p-4 lg:px-8" aria-label="Global">
        <div className="flex lg:flex-1">
          <a className="-m-1.5 p-1.5 text-sm font-semibold leading-6 text-gray-900">SRWA - Yield Derivatives</a>
        </div>
        <div className="lg:flex lg:flex-1 lg:justify-end">
          <radix-connect-button></radix-connect-button>
        </div>
      </nav>
    </header>
  );
}

export default AppHeader;
