import { useState, useEffect } from 'react';

const useWebsocket = (url: string) => {
    const [socket, setSocket] = useState<WebSocket | null>(null);
    const [message, setMessage] = useState<string | null>(null);

    useEffect(() => {
        const ws = new WebSocket(url);
        
        ws.addEventListener('open', (e) => {
            console.log('Connection established');
        });

        ws.addEventListener('message', (e) => {
            console.log('Message received');
            setMessage(e.data);
        });

        ws.addEventListener('close', (e) => {
            console.log('Connection closed');
        });

        ws.addEventListener('error', (e) => {
            console.error('Error occurred');
            console.error(e);
        });

        setSocket(ws);

        // Cleanup when unmounting
        return () => {
            ws.close();
        };
    }, [url]);

    const send = (msg: string) => {
        if (!socket) {
            console.error('No socket connection');
            return;
        }
        socket.send(msg);
    }

    return { message, send };

}

export default useWebsocket;
