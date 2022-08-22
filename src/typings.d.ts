type Task = {
  url: string;
  output: string;
  createAt?: string;
  status?: number | string;
};

type GlobalConfig = {
  thread?: number;
  proxyAddress?: string;
};
