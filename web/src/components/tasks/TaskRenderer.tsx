import { Task } from "@/types/Task"
import { useMutation } from "react-query"

type Props = {
    task: Task
    refetch: () => Promise<unknown>
}

const TaskRenderer: React.FC<Props> = ({ task, refetch }) => {
    const completeMutation = useMutation('completeTask', async (data: {id: number, state: boolean}) => {
        const _resp = await fetch(`http://127.0.0.1:3001/api/tasks?id=${data.id}&state=${data.state}`, {
            method: 'PATCH'
        })
    }, {
        onSuccess: () => {
            refetch()
        }
    })

    const deleteMutation = useMutation('deleteTask', async (id: number) => {
        const _resp = await fetch(`http://127.0.0.1:3001/api/tasks?id=${id}`, {
            method: 'DELETE'
        })
    }, {
        onSuccess: () => {
            refetch()
        }
    })

    return (
        <div className="text-2xl flex space-x-2 px-4 py-2 bg-neutral-900 w-1/3 rounded-lg">
            <input type="checkbox" checked={task.completed} onClick={() => completeMutation.mutate({ id: task.id, state: !task.completed })} />
            <span className="w-full">{task.name}</span>
            <button className="text-base" onClick={() => deleteMutation.mutate(task.id)}>Wykasuj</button>
        </div>
    )
}

export default TaskRenderer;