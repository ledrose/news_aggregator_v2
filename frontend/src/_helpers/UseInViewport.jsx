import { useEffect, useMemo, useState } from "react";

export default function useInViewport(ref) {
    const [isInteresting,setIsInteresting] = useState(false);
    const observer = useMemo(
        () => 
            new IntersectionObserver(([entry]) => 
                setIsInteresting(entry.isIntersecting),    
            ),[]);
    useEffect(() => {
        observer.observe(ref.current);
        return () => {
            observer.disconnect();
        }
    },[ref,observer])
    return isInteresting;
}