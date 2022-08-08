import {
  Loading,
  Progress,
  Table,
  Tooltip,
  Text,
  Spacer,
} from '@nextui-org/react';
import React, { ReactNode } from 'react';
import { useSnapshot } from 'valtio';
import { columns, TaskStatus } from './config';
import model from './model';
import { StyledBadge } from './StyledBadge';

function renderOutput(task: Task) {
  return (
    <Tooltip content={task.url} color="secondary">
      <Text
        css={{
          textGradient: '45deg, $purple600 -20%, $pink600 100%',
        }}
      >
        {task.output}
      </Text>
    </Tooltip>
  );
}

function renderTaskStatus(status?: number | TaskStatus) {
  if (status === undefined || status === 0) {
    return <Loading size="sm" />;
  }
  if (status === TaskStatus.successful) {
    return <StyledBadge type="successful">已完成</StyledBadge>;
  }
  if (typeof status === 'string') {
    return (
      <Tooltip content={status} color="error">
        <StyledBadge type="failed">下载失败</StyledBadge>
      </Tooltip>
    );
  }
  return (
    //
    //{' '}
    <div>
      <Text>{`${(status * 100).toFixed(2)}%`}</Text>
      <Progress value={Number(status) * 100} color="gradient" />
    </div>
  );
}

const TaskTable: React.FC = () => {
  const state = useSnapshot(model);
  return (
    <Table
      aria-label="Example table with dynamic content"
      fixed
      css={{
        height: 'auto',
        minWidth: '100%',
      }}
    >
      <Table.Header columns={columns}>
        {column => (
          <Table.Column key={column.key} css={{ width: column.width }}>
            {column.label}
          </Table.Column>
        )}
      </Table.Header>
      <Table.Body>
        {state.list.map((task, index) => (
          <Table.Row key={index}>
            {columnKey => {
              let v: ReactNode = task[columnKey as keyof typeof task];
              if (columnKey === 'status') {
                v = renderTaskStatus(v as TaskStatus);
              }
              if (columnKey === 'output') {
                v = renderOutput(task);
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
