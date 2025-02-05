try {
    rs.status()
} catch (err) {
    rs.initiate({
        _id: 'rs0',
        members: [
            { _id: 0, host: 'mongodb:27017', priority: 1 },
            { _id: 1, host: 'mongodb_secondary:27017', priority: 0.5 },
        ],
    })
}
