import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";
import { useLoaderData } from "react-router";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export const clientLoader = async () => {
  const data = await fetch(import.meta.env.VITE_API_URL);
  const d = data.json();
  return d;
};

export default function Home({ loaderData }: Route.ComponentProps) {
  console.dir(loaderData);
  return <Welcome />;
}
