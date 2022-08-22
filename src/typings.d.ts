type Task = {
  url: string;
  output: string;
  createAt?: string;
  status?: number | 'waiting';
};

type GlobalConfig = {
  thread?: number;
  proxyAddress?: string;
};
