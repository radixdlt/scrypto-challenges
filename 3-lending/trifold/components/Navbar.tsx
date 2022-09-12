import Link from "next/link";

const Navbar = () => {
  // a floating bottom navigation menu
  return (
    <nav className="bottom-4 fixed text-white mx-auto w-full my-5">
      <div className="flex flex-row bg-nordhighlight px-5 py-2 mx-10 rounded-lg shadow-xl justify-around">
        <ColoredLink href="/" name="Home" />
        <ColoredLink href="/about" name="About" />
        <ColoredLink href="/admin" name="Admin" />
        <ColoredLink href="/borrow" name="Borrow" />
        <ColoredLink href="/lend" name="Lend" />
      </div>
    </nav>
  );
};

export default Navbar;

interface LinkProps {
  href: string;
  name: string;
}

const ColoredLink = ({ href, name }: LinkProps) => {
  return (
    <Link href={href}>
      <a className="text-white hover:bg-[#425A79] py-3 px-5 rounded-md">
        {name}
      </a>
    </Link>
  );
};
