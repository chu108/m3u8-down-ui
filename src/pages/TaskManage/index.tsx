import { Text } from '@nextui-org/react';
import React from 'react';
import CreateTask from './CreateTask';
import GlobalConfig from './GlobalConfig';
import RemoveHistory from './RemoveHistory';
import TaskTable from './TaskTable';

const iStyle = {
  display: 'flex',
  justifyContent: 'space-between',
  alignItems: 'center',
};

const TaskManage: React.FC = () => {
  return (
    <main style={{ padding: 10 }}>
      <div style={iStyle}>
        <div>
          <Text
            h4
            css={{
              marginLeft: 10,
              fontSize: 20,
              textGradient: '45deg, $blue600 -20%, $purple600 100%',
            }}
          >
            下载历史
          </Text>
        </div>
        <div style={iStyle}>
          <CreateTask />
          <GlobalConfig />
          <RemoveHistory />
        </div>
      </div>
      <TaskTable />
    </main>
  );
};

export default TaskManage;
