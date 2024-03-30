"use client"

import { useEffect } from 'react';

function TailwindClient() {
    useEffect(() => {
        // require('bootstrap/dist/js/bootstrap.bundle.min.js');
        require('flowbite/dist/flowbite.min.js')
        // <script src="../path/to/flowbite/dist/flowbite.min.js"></script>
    }, []);

    return null;
}

export default TailwindClient;