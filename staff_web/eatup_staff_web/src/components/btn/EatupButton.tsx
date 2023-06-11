interface Props {
  type?: "primary" | "secondary" | "success" | "danger" | "warning" | "info" | "light" | "dark";
  extraClasses?: string[];
  onClick: () => void;
  children: any;
};

const EatupButton = ({type, extraClasses, onClick, children}: Props) => {
  type = type || "dark";
  extraClasses = extraClasses || [];
  return (
    <button 
      className={`btn btn-${type} w-100 ${extraClasses?.join(" ")}`}
      onClick={onClick}
    >{children}</button>
  );
};

export default EatupButton;