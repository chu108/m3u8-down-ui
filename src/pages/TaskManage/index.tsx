import { Text } from "@nextui-org/react";
import React from "react";
import CreateTask from "./CreateTask";
import TaskTable from "./TaskTable";

const TaskManage: React.FC = () => {
  return (
    <main style={{ padding: 10 }}>
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <div>
          <Text
            h4
            css={{
              marginLeft: 10,
              textGradient: "45deg, $blue600 -20%, $purple600 100%",
            }}
          >
            下载记录
          </Text>
        </div>
        <div>
          <CreateTask />
        </div>
      </div>
      <TaskTable />
    </main>
  );
};

export default TaskManage;
