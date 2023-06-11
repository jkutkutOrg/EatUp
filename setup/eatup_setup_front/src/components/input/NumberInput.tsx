import TextInput from "./TextInput";

interface Props {
  id: string;
  placeholder?: string;
}

const NumberInput = ({id, placeholder = ""}: Props) => {
  const onBeforeInput = (e: any) => {
    if (e.data.match(/[^0-9]/)) e.preventDefault();
  };
  return <TextInput id={id} placeholder={placeholder} onBeforeInput={onBeforeInput}/>;
};

export default NumberInput;