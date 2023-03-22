import React from "react";
import Layout from "../components/Layout";
import { useForm } from "react-hook-form";
import Input from "../components/Input";

interface Props {}

function Savings(props: Props) {
  const { register, handleSubmit } = useForm();
  const onSubmit = (data: any) => console.log(data);
  const {} = props;

  return (
    <Layout>
      <strong className="text-gray-400">Savings Preference</strong>
      <main className="my-4">
        <form className="flex flex-col gap-2 w-fit" onSubmit={handleSubmit(onSubmit)}>
          <h1 className="text-2xl"> Let's get to know your investment goal</h1>
          <Input
            label="What is your Saving goal"
            type={"text"}
            placeholder={"Sample text"}
            onChange={(e) => console.log(e.target.value)}
          />

          <Input
            label="What is your goal"
            type={"text"}
            placeholder={"Sample text"}
            onChange={(e) => console.log(e.target.value)}
          />

          <Input
            label="What is your goal"
            type={"text"}
            placeholder={"Sample text"}
            onChange={(e) => console.log(e.target.value)}
          />
        </form>
      </main>
    </Layout>
  );
}

export default Savings;
