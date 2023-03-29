interface Props {
  text: React.ReactNode;
  variant?: "primary" | "secondary";
  type?: "button" | "submit" | "reset";
}

function DefinedButton(props: Props) {
  const { text, variant } = props;
  const btnStyle =
    variant === "primary"
      ? "bg-primary-1 flex items-center gap-1 text-black font-medium px-4 py-2 rounded-md  hover:shadow-sm hover:shadow-primary-1"
      : " text-primary-1 fill-primary-1 flex  items-center gap-1 border-primary-1 border-2 font-medium px-4 py-2 rounded-md hover:shadow-md hover:shadow-primary-1";
  return <button className={btnStyle}>{text}</button>;
}

export default DefinedButton;
