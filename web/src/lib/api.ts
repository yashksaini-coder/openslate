const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3001";

export async function api(path: string, init?: RequestInit): Promise<Response> {
  const res = await fetch(`${API_URL}${path}`, {
    ...init,
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...init?.headers,
    },
  });

  return res;
}
