export type Item = {
    id: number
    title: string
    done: boolean
}

export type Props = {
    items: Item[]
    markItem: (id: number) => void
    unmarkItem: (id: number) => void
    deleteItem: (id: number) => void
    loading: boolean
}

function CrossButton({ onClick }: { onClick: () => void }) {
    return (
        <button
            className="bg-amber-700 hover:bg-amber-900 text-white content-center w-7 h-7 rounded-lg"
            onClick={onClick}
        >
            x
        </button>
    )
}

export function TodoList(props: Props) {
    return (
        <ul className="flex flex-col max-w-fit p-4 gap-2">
            {props.items.map((item) => (
                <div
                    key={item.id}
                    className="
                    px-5 py-2
                    border rounded border-slate-400 hover:shadow cursor-pointer
                    flex flex-row gap-3
                    "
                >
                    <input
                        type="checkbox"
                        checked={item.done}
                        onChange={() =>
                            item.done
                                ? props.unmarkItem(item.id)
                                : props.markItem(item.id)
                        }
                        disabled={props.loading}
                    />
                    <span>{item.title}</span>
                    <CrossButton onClick={() => props.deleteItem(item.id)} />
                </div>
            ))}
        </ul>
    )
}
