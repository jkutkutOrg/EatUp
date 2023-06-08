interface Props {
  extraClasses?: string[];
  onClick: () => void;
  children: any;
};

const EatupButton = ({extraClasses, onClick, children}: Props) => {
  return (
    <button 
      className={`btn btn-primary btn-dark w-100 ${extraClasses?.join(" ")}`}
      onClick={onClick}
    >{children}</button>
  );
};

export default EatupButton;