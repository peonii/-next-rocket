import { useState } from "react";
import { useMutation } from "react-query";

type Props = {
    refetch: () => Promise<unknown>
}

const TaskCreateForm: React.FC<Props> = ({ refetch }) => {
    const [name, setName] = useState('');

    const createTaskMutation = useMutation('createTaskMutation', async () => {
        await fetch('http://127.0.0.1:3001/api/tasks', {
            method: 'POST',
            body: JSON.stringify({
                name
            })
        })

        setName('');
    }, {
        onSuccess: () => {
            refetch();
        }
    })

    return (
        <div className="bg-neutral-900 rounded-lg flex space-x-4 mt-6 ml-6 w-1/3 p-4">
            <input 
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="bg-neutral-700 p-2 rounded-lg w-full"
            />
            <button className="font-bold" onClick={() => createTaskMutation.mutate()}>
                +
            </button>
        </div>
    )
}

export default TaskCreateForm;