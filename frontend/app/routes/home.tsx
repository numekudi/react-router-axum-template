import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";
import type { components, paths } from "~/lib/api/v1";
import { client } from "~/lib/client";
export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export const clientLoader = async () => {
  const d = await client.GET("/hello");
  return d.data;
};

export default function Home({ loaderData }: Route.ComponentProps) {
  console.log(loaderData);
  return <Welcome />;
}
