import { useState } from "react"

export type Item = {
    id: number
    title: string
    done: boolean
}

type Actions = {
    markItem: (id: number) => void
    unmarkItem: (id: number) => void
    deleteItem: (id: number) => void
    createItem: (title: string) => void
}

export type Props = Actions & {
    items: Item[]
    loading: boolean
}

export function TodoList({ items, loading, ...actions }: Props) {
    const [draft, setDraft] = useState("")

    function createItem() {
        actions.createItem(draft)
        setDraft("")
    }

    return (
        <div className="p-2 flex flex-col">
            <div className="flex flex-row gap-2">
                <input
                    className="border border-slate-600 px-2 py-0.5 w-80"
                    type="text"
                    value={draft}
                    onChange={(e) => setDraft(e.target.value)}
                    placeholder="Buy groceries"
                />
                <button
                    className="
                    rounded-lg
                    px-3 py-1
                    bg-slate-300 text-blue-900
                    hover:bg-slate-200"
                    onClick={createItem}
                >
                    Add
                </button>
            </div>
            <ul className="flex flex-col max-w-fit p-4 gap-2">
                {items.map((item) => (
                    <TodoItem
                        key={item.id}
                        {...{ item, loading, ...actions }}
                    />
                ))}
            </ul>
        </div>
    )
}

function DeleteButton({ onClick }: { onClick: () => void }) {
    return (
        <button
            className="bg-slate-300 hover:bg-slate-200 text-blue-900 content-center rounded-lg px-3 py-1"
            onClick={onClick}
        >
            Delete
        </button>
    )
}

function TodoItem(props: { item: Item; loading: boolean } & Actions) {
    return (
        <div
            className="
        px-5 py-2
        border rounded border-slate-400 hover:shadow cursor-pointer
        flex flex-row gap-3
        "
        >
            <input
                type="checkbox"
                checked={props.item.done}
                onChange={() =>
                    props.item.done
                        ? props.unmarkItem(props.item.id)
                        : props.markItem(props.item.id)
                }
                disabled={props.loading}
            />
            <span>{props.item.title}</span>
            <DeleteButton onClick={() => props.deleteItem(props.item.id)} />
        </div>
    )
}
