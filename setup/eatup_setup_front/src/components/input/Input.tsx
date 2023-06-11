interface Props {
  id: string;
  type: string;
  placeholder?: string;
  onBeforeInput?: (e: any) => void;
}

const Input = ({
  id,
  type,
  placeholder = "",
  onBeforeInput = (e: any) => {}
}: Props) => {
  const resetValidation = (e: any) => {
    e.target.classList.remove("is-invalid");
    e.target.classList.remove("is-valid");
  };

  return <input
    id={id} type={type}
    className="form-control"
    placeholder={placeholder}
    onBeforeInput={onBeforeInput}
    onInput={resetValidation}
  />;
};

export default Input;