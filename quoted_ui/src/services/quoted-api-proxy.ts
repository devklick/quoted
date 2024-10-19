import axios, {
  AxiosError,
  AxiosResponse,
  InternalAxiosRequestConfig,
} from "axios";

const baseURL =
  import.meta.env.MODE === "development"
    ? "/api"
    : import.meta.env.VITE_BASE_URL;

export const proxy = axios.create({ baseURL });

// Adding interceptors

proxy.interceptors.request.use(transformDataToSnakeCase);

proxy.interceptors.response.use(
  transformResponseKeysToCamelCase,

  transformErrorKeysToCamelCase
);

// To transform response data to camel case

function transformResponseKeysToCamelCase(response: AxiosResponse) {
  if (response.data) {
    response.data = keysToCamelCase(response.data);
  }

  return response;
}

// To transform error response data to camel case

function transformErrorKeysToCamelCase(error: AxiosError) {
  if (error.response?.data) {
    error.response.data = keysToCamelCase(error.response.data);
  }

  return error;
}

// To transform the request payload to snake_case

function transformDataToSnakeCase(request: InternalAxiosRequestConfig) {
  request.data = keysToSnakeCase(request.data);
  request.params = keysToSnakeCase(request.params);
  return request;
}

function transformObjectDeep<
  T,
  F extends (key: string, value: unknown) => [string, unknown]
>(object: T, keyValueTransformer: F): unknown {
  if (Array.isArray(object)) {
    return object.map((obj) => transformObjectDeep(obj, keyValueTransformer));
  } else if (object === null || typeof object !== "object") {
    return object;
  }

  return Object.fromEntries(
    Object.entries(object).map(([key, value]) =>
      keyValueTransformer(key, transformObjectDeep(value, keyValueTransformer))
    )
  );
}

function keysToCamelCase(object: unknown) {
  return transformObjectDeep(object, (key, value) => [
    snakeToCamelCase(key),
    value,
  ]);
}

function keysToSnakeCase(object: unknown) {
  return transformObjectDeep(object, (key, value) => [
    camelToSnakeCase(key),
    value,
  ]);
}

function snakeToCamelCase(value: string) {
  return value.replace(/(_\w)/g, (letter) => letter[1].toUpperCase());
}

function camelToSnakeCase(value: string) {
  return value.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}
