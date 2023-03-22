import React, { useState } from "react";
import { AiFillFund, AiFillAppstore, AiFillSetting } from "react-icons/ai";
interface Props {}

const iconStyle1 = `flex items-center justify-center gap-2 w-fit p-2 hover:bg-gray-700 rounded-md cursor-pointer `;
const iconStyle2 = `flex items-center justify-center gap-2 w-fit p-2 fill-dark text-black bg-primary-1 rounded-md cursor-pointer `;
export const DashboardIcon = (props: Props) => {
  const {} = props;
  const [active, setActive] = useState(false);

  return (
    <div className={active ? iconStyle1 : iconStyle2}>
      Dashboard
      <AiFillFund />
    </div>
  );
};

export const IntegrationIcon = (props: Props) => {
  const {} = props;

  return (
    <div className={iconStyle1}>
      Integration
      <AiFillAppstore />
    </div>
  );
};
export const PreferencesIcon = (props: Props) => {
  const {} = props;

  return (
    <div className={iconStyle1}>
      Preferences
      <AiFillSetting />
    </div>
  );
};
