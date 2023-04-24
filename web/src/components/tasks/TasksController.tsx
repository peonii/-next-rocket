import { useQuery } from "react-query";
import TasksRenderer from "./TasksRenderer";
import { Tasks } from "@/types/Task";
import TaskCreateForm from "../create/TaskCreateForm";

const TasksController: React.FC = () => {
    const tasksQuery = useQuery('tasks', async () => {
        const resp = await fetch('http://127.0.0.1:3001/api/tasks');

        const respJson = (await resp.json()) as Tasks;

        return respJson
    })

    if (tasksQuery.isLoading) {
        return <div>...</div>
    }

    if (tasksQuery.isError) {
        return <div>The request failed!</div>
    }

    if (!tasksQuery.data) {
        return <div>No data found!</div>
    }

    return (
        <>
            <TasksRenderer refetch={tasksQuery.refetch} tasks={tasksQuery.data} />
            <TaskCreateForm refetch={tasksQuery.refetch} />
        </>
    )
}

export default TasksController;