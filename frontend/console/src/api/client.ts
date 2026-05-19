const API_BASE_URL =
  import.meta.env.VITE_API_BASE_URL ?? "http://localhost:3000";

export class ApiClientError extends Error {
  readonly status: number;
  readonly body: string;

  constructor(path: string, status: number, body: string) {
    super(`GET ${path} failed: ${status} ${body}`);

    this.name = "ApiClientError";
    this.status = status;
    this.body = body;
  }
}

export async function apiGet<T>(path: string): Promise<T> {
  const response =
    await fetch(`${API_BASE_URL}${path}`, {
      headers: {
        Accept: "application/json",
      },
    });

  if (!response.ok) {
    const body =
      await response.text();

    throw new ApiClientError(path, response.status, body);
  }

  return response.json() as Promise<T>;
}

export async function apiPost<TResponse, TRequest>(
  path: string,
  body: TRequest,
): Promise<TResponse> {
  const response =
    await fetch(`${API_BASE_URL}${path}`, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });

  if (!response.ok) {
    const responseBody =
      await response.text();

    throw new ApiClientError(path, response.status, responseBody);
  }

  return response.json() as Promise<TResponse>;
}
