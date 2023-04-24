import { Tasks } from "@/types/Task";
import TaskRenderer from "./TaskRenderer";

type Props = {
    tasks: Tasks;
    refetch: () => Promise<unknown>;
}

const TasksRenderer: React.FC<Props> = ({ tasks, refetch }) => {
    return (
        <div className="flex flex-col space-y-2 mt-6 ml-6">
            {tasks.map(task => (
                <TaskRenderer key={task.id} task={task} refetch={refetch} />
            ))}
        </div>
    )
}

export default TasksRenderer;