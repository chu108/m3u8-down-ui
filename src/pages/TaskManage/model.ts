import { proxy } from "valtio";
import { subscribeKey } from "valtio/utils";
import { rows } from "./config";
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

let initialData = {
  newTask: null as Task | null,
  list: rows as Task[],
};
try {
  const data = localStorage.getItem("TASK_STATE");
  if (data) {
    initialData = Object.assign(initialData, JSON.parse(data));
  }
} catch (error) {
  console.error(error);
}

const model = proxy(initialData);

function setTaskList(list: Task[]) {
  model.list = list;
}

subscribeKey(model, "list", (list) => {
  console.log("task list ", list);
  let hasNewTask = false;
  const newList = list.map((task) => {
    if (task.status === undefined) {
      hasNewTask = true;
      invoke("down_file", { filePath: task.url, output: task.output }).then(
        (response) => console.log(response)
      );
      task.status = 0;
    }
    return task;
  });
  if (hasNewTask) {
    setTaskList(newList);
  }
});

listen("downing", (event) => {
  console.log("downing event", event);
  try {
    const res = JSON.parse((event.payload as any)?.message);
    console.log("downing res", res);
    const { url, total, downloaded } = res;

    if (url && total && downloaded) {
      const newPercent = Math.floor((downloaded / total) * 100);
      console.log("newPercent", newPercent);
      let hasTaskUpdate = false;
      const newList = model.list.map((task) => {
        console.log("url", url);
        if (task.url === url) {
          const percent = Number(task.status);
          console.log("percent", percent);
          if (percent < 100 && percent >= 0) {
            hasTaskUpdate = newPercent !== percent;
            task.status = newPercent;
          }
        }
        return task;
      });
      console.log("hasTaskUpdate", hasTaskUpdate);
      if (hasTaskUpdate) {
        model.list = newList;
      }
    }
  } catch (error) {
    console.error(error);
  }
});

export default model;
