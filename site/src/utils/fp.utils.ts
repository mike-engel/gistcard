export const propOr = (fallback: any, property: string) => (obj: Record<string, any>) =>
  obj[property] || fallback;
