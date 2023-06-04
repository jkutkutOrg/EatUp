import Input from "./Input";

interface Props {
    id: string;
    placeholder?: string;
    onBeforeInput?: (e: any) => void;
}

const TextInput = ({
    id,
    placeholder = "",
    onBeforeInput = (e: any) => {}
}: Props) => {
    return <Input id={id} type="text" placeholder={placeholder} onBeforeInput={onBeforeInput}/>;
};

export default TextInput;