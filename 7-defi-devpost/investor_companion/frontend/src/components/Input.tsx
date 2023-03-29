import React from "react";

interface Props {
  label?: string;
  type: string;
  placeholder: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}

function Input(props: Props) {
  const { label, type, placeholder, onChange } = props;

  return (
    <div className="flex flex-col gap-2">
      <label>{label}</label>
      <input className="p-2 outline-none text-sm bg-gray-600 rounded-sm" type={type} placeholder={placeholder} onChange={onChange} />
    </div>
  );
}

export default Input;
