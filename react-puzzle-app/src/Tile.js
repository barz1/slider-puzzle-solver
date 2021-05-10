import Button from '@material-ui/core/Button';
import { withStyles } from '@material-ui/core';
import { useState } from 'react';
import clsx from 'clsx';

const styles = {
    root: {
        width: '150px',
        height: '150px',
        fontSize: 'xx-large',
    },
    blank: {
        background: 'white',
        color: 'white',

        '&:hover': {
            background: 'white',
            color: 'white',
            cursor: 'default'
        }
    }
};

function Tile(props) {
    const style = props.value === 0 ? clsx(props.classes.blank, props.classes.root, props.className)
                                    : clsx(props.classes.root, props.className);


    const handleClick = () => {
        props.handleClick(props.index);
    }

    return <Button key={props.index} onClick={handleClick} disableRipple={true} variant='contained' className={style} color='primary'>
                {props.value}
            </Button>
}

export default withStyles(styles)(Tile);