export type Stylable = { className?: string };

export type ReducerAction<T = any, P = any> = {
  type: T;
  payload?: P;
};
