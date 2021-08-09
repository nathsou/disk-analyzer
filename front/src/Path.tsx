import React, { FC, useMemo } from "react";
import { Link, useLocation } from "wouter";

export const Path: FC<{ path: string }> = ({ path }) => {
  const sections = useMemo(() => (
    path.split('/').slice(1).reduce<Array<{ name: string, path: string }>>((prev, name) => {
      const { path } = prev[prev.length - 1] ?? { name: '', path: '' };
      return [...prev, { path: `${path}/${name}`, name }];
    }, [])
  ), [path]);

  return (
    <div style={{ display: 'flex' }}>
      {sections.map(({ path, name }, index) => (
        <div key={index}>
          <Section name={name} path={path} />
          {index < sections.length - 1 ? <Separator /> : null}
        </div>
      ))}
    </div>
  );
};

const Separator = () => {
  return <span style={{ margin: '0 4px' }}>{'>'}</span>;
};

const Section: FC<{ name: string, path: string }> = ({ name, path }) => {
  const [location] = useLocation();
  const isActive = location.endsWith(path);

  return (
    <Link
      className='breadcrumb-link'
      style={{ fontWeight: isActive ? 700 : 500 }}
      href={`/ls${path}`}
    >
      {name}
    </Link>
  );
};