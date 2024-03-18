CREATE TABLE blog (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  slug VARCHAR(200) NOT NULL,
  content TEXT NOT NULL,
  image_link TEXT NOT NULL,
  thumbnail_link TEXT NOT NULL,
  featured BOOLEAN NOT NULL,
  created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
  edited TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO blog (title, slug, content, image_link, thumbnail_link, featured, created, edited) VALUES
('My First Blog Post', 'my-first-blog-post', 'This is the content of my first blog post.', 'https://example.com/image1.jpg', 'https://example.com/thumb1.jpg', TRUE, '2024-3-27T12:34:56Z', '2024-3-27T12:34:56Z'),
('My Second Blog Post', 'my-second-blog-post', 'This is the content of my second blog post.', 'https://example.com/image2.jpg', 'https://example.com/thumb2.jpg', FALSE, '2024-3-28T12:34:56Z', '2024-3-28T12:34:56Z'),
('My Third Blog Post', 'my-third-blog-post', 'This is the content of my third blog post.', 'https://example.com/image3.jpg', 'https://example.com/thumb3.jpg', TRUE, '2024-3-29T12:34:56Z', '2024-3-29T12:34:56Z');
