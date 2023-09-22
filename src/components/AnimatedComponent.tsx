import { Spring, animated } from 'react-spring';

const AnimatedComponent = {
  OpacityFadeInDiv: ({ children, delay }: any) => (
    <Spring
      delay={delay}
      from={{
        opacity: 0,
      }}
      to={{
        opacity: 1,
      }}>
      {(styles: any) => <animated.div style={styles}>{children}</animated.div>}
    </Spring>
  ),
};

export default AnimatedComponent;
