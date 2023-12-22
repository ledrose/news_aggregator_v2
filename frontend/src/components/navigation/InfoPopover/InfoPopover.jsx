import { useEffect, useRef, useState } from "react";
import { Button, Overlay, OverlayTrigger, Popover, Tooltip } from "react-bootstrap";
import './InfoPopover.css';

export default function InfoPopover({errorState}) {
    // const text = "Lorem ipsum fjwlefjwlekfjwlkfjewlkfjwlkfjwelkfjwelkfwjeflkwejflk";
    const target = useRef(null);
    const [err,setErr] = errorState;
    // const [show,setShow] = useState(false);
    useEffect(() => {
        if (err!=null) {
            const timer = setTimeout(()=>{setErr(null)},2000);
        }
    },[err])
    const errText = (err!=null)?err.toString():"";
    return (
        <>
            {/* <Button ref={target} variant="primary" onClick={()=>setShow(true)}>Show modal</Button> */}
            <div ref={target}></div>
            <Overlay target={target.current} show={err!=null} placement="bottom">
                {(props) => (
                    <Popover id="popover-basic" className="popup-error">
                        <Popover.Header as="h3">Error</Popover.Header>
                        <Popover.Body>
                            {errText}
                        </Popover.Body>
                    </Popover>
                    // <Tooltip className="popup-error" id="popup-1">
                    //     {errText}
                    // </Tooltip>
                )}
            </Overlay>
        </>
    )
}
