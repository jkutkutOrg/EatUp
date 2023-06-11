import Input from "./Input";

interface Props {
  id: string;
  placeholder?: string;
}

const PasswordInput = ({id, placeholder = ""}: Props) => {
  return <Input id={id} type="password" placeholder={placeholder}/>;
};

export default PasswordInput;