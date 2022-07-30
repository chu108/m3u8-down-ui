import { Loading, Progress, Table, Text } from "@nextui-org/react";
import React, { ReactNode } from "react";
import { useSnapshot } from "valtio";
import { columns, TaskStatus } from "./config";
import model from "./model";
import { StyledBadge } from "./StyledBadge";

function renderTaskStatus(status?: number | TaskStatus) {
  switch (status) {
    case undefined:
    case 0:
      return <Loading size="sm" />;
    case TaskStatus.successful:
      return <StyledBadge type="successful">已完成</StyledBadge>;
    case TaskStatus.failed:
      return <StyledBadge type="failed">下载失败</StyledBadge>;
    default:
      if (status < 100) {
        return <Progress value={Number(status)} color="gradient" />;
      }
      return status;
  }
}

const TaskTable: React.FC = () => {
  const state = useSnapshot(model);
  return (
    <Table
      aria-label="Example table with dynamic content"
      fixed
      css={{
        height: "auto",
        minWidth: "100%",
      }}
    >
      <Table.Header columns={columns}>
        {(column) => (
          <Table.Column key={column.key} css={{ width: column.width }}>
            {column.label}
          </Table.Column>
        )}
      </Table.Header>
      <Table.Body>
        {state.list.map((task, index) => (
          <Table.Row key={index}>
            {(columnKey) => {
              let v: ReactNode = task[columnKey as keyof typeof task];
              if (columnKey === "status") {
                v = renderTaskStatus(v as TaskStatus);
              }
              return <Table.Cell>{v}</Table.Cell>;
            }}
          </Table.Row>
        ))}
      </Table.Body>
      <Table.Pagination
        shadow
        noMargin
        align="center"
        total={Math.ceil(model.list.length / 8)}
        rowsPerPage={8}
      />
    </Table>
  );
};

export default TaskTable;
