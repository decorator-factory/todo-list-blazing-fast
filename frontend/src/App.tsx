import { TodoList, Item } from "./TodoList"
import * as remote from "./remote"
import { useEffect, useState } from "react"

const baseUrl = "/api"

export function App() {
    const [items, setItems] = useState<Item[]>([])
    const [loading, setLoading] = useState(false)

    const reloadItems = () => {
        setLoading(true)

        const abort = new AbortController()
        remote
            .fetchTodos(baseUrl, abort.signal)
            .then((answer) => {
                setLoading(false)
                if (answer.status === "ok") {
                    setItems(answer.payload)
                } else {
                    console.error(
                        `Got error on GET /api/todos: ${JSON.stringify(
                            answer,
                        )}`,
                    )
                }
            })
            .catch((reason) => {
                setLoading(false)
                console.warn(reason)
            })

        return () => abort.abort()
    }

    useEffect(reloadItems, [])

    const markItem = async (id: number) => {
        setLoading(true)
        await remote.markTodo(baseUrl, id)
        reloadItems()
    }

    const unmarkItem = async (id: number) => {
        setLoading(true)
        await remote.unmarkTodo(baseUrl, id)
        reloadItems()
    }

    const deleteItem = async (id: number) => {
        setLoading(true)
        await remote.deleteTodo(baseUrl, id)
        reloadItems()
    }

    return (
        <TodoList
            loading={loading}
            items={items}
            markItem={markItem}
            unmarkItem={unmarkItem}
            deleteItem={deleteItem}
        />
    )
}
